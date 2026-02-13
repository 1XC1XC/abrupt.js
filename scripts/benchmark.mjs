import crypto from "node:crypto"
import fs from "node:fs"
import path from "node:path"

import abrupt from "../index.mjs"
import Benchmark from "./profile.mjs"

const { rand, crypto: abruptCrypto, file } = abrupt
const { base64 } = abruptCrypto
const profile = new Benchmark()

const FAST_ITERS = 100_000
const MEDIUM_ITERS = 50_000
const HEAVY_ITERS = 25_000
const FILE_ITERS = 5_000
const WARMUP_ITERS = 10_000

const LETTERS = "abcdefghijklmnopqrstuvwxyz"
const ALL_CHARS =
	"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?/`~"

const SAMPLE_ARRAY = Array.from({ length: 64 }, (_, i) => i + 1)
const SAMPLE_OBJECT = Object.fromEntries(SAMPLE_ARRAY.map((v) => [`k${v}`, v]))
const SAMPLE_OBJECT_KEYS = Object.keys(SAMPLE_OBJECT)

const HASH_INPUT = "The quick brown fox jumps over the lazy dog."
const BASE64_INPUT = "Hello World! abrupt.rs napi benchmark"
const ROT_INPUT = "Hello World! 123"

const REPO_ROOT = process.cwd()
const BENCH_ROOT_REL = "benchmark/.sandbox"
const BENCH_ROOT_ABS = path.resolve(REPO_ROOT, BENCH_ROOT_REL)
const BENCH_DIR_ABS = path.resolve(REPO_ROOT, "benchmark")
const TMP_DIR_ABS = path.resolve(REPO_ROOT, ".tmp")
const BENCH_FILE_REL = `${BENCH_ROOT_REL}/sample.txt`
const BENCH_DIR_REL = `${BENCH_ROOT_REL}/dir`

function consume(value) {
	if (typeof value === "number") return Number.isFinite(value) ? value : 0
	if (typeof value === "string") return value.length
	if (typeof value === "boolean") return value ? 1 : 0
	if (Array.isArray(value)) return value.length
	return value == null ? 0 : 1
}

function isWithin(base, target) {
	const basePath = path.resolve(base)
	const targetPath = path.resolve(target)
	if (basePath === targetPath) return true
	return targetPath.startsWith(`${basePath}${path.sep}`)
}

function safeBenchmarkPath(relativePath) {
	if (path.isAbsolute(relativePath)) {
		throw new Error("Benchmark path must be relative")
	}

	const resolved = path.resolve(REPO_ROOT, relativePath)
	if (!isWithin(REPO_ROOT, resolved)) {
		throw new Error("Benchmark path escaped repository root")
	}
	if (!isWithin(BENCH_ROOT_ABS, resolved)) {
		throw new Error("Benchmark path escaped benchmark sandbox")
	}

	return resolved
}

function safeRepoDirectory(targetPath) {
	const resolved = path.resolve(targetPath)
	if (!isWithin(REPO_ROOT, resolved)) {
		throw new Error("Cleanup path escaped repository root")
	}
	return resolved
}

function resetBenchSandbox() {
	const root = safeBenchmarkPath(BENCH_ROOT_REL)
	fs.rmSync(root, { recursive: true, force: true })
	fs.mkdirSync(root, { recursive: true })
}

function cleanupBenchSandbox() {
	const root = safeBenchmarkPath(BENCH_ROOT_REL)
	fs.rmSync(root, { recursive: true, force: true })
}

function cleanupRunArtifacts() {
	cleanupBenchSandbox()
	fs.rmSync(safeRepoDirectory(BENCH_DIR_ABS), { recursive: true, force: true })
	fs.rmSync(safeRepoDirectory(TMP_DIR_ABS), { recursive: true, force: true })
}

function js_int(min, max) {
	const lo = Math.min(min, max)
	const hi = Math.max(min, max)
	return Math.floor(Math.random() * (hi - lo + 1)) + lo
}

function js_float(min, max) {
	const lo = Math.min(min, max)
	const hi = Math.max(min, max)
	return Math.random() * (hi - lo) + lo
}

function js_str(length = 5, lettersOnly = false) {
	const charset = lettersOnly ? LETTERS : ALL_CHARS
	let output = ""
	for (let i = 0; i < length; i++) {
		output += charset[(Math.random() * charset.length) | 0]
	}
	return output
}

function js_bool() {
	return Math.random() >= 0.5
}

function js_array(values) {
	return values[(Math.random() * values.length) | 0]
}

function js_object(values, key = false) {
	const keys = Object.keys(values)
	const picked = keys[(Math.random() * keys.length) | 0]
	return key ? picked : values[picked]
}

function js_object_with_keys(values, keys, key = false) {
	const picked = keys[(Math.random() * keys.length) | 0]
	return key ? picked : values[picked]
}

function js_base64_encode(input) {
	return Buffer.from(input, "utf8").toString("base64")
}

function js_base64_decode(input) {
	return Buffer.from(input, "base64").toString("utf8")
}

function js_sha256(input, encoding = "hex") {
	return crypto.createHash("sha256").update(input).digest(encoding)
}

function js_rot(input, shift = 13) {
	let s = shift % 26
	if (s === 0) s = 13
	let out = ""
	for (const ch of input) {
		const code = ch.charCodeAt(0)
		if (code >= 65 && code <= 90) {
			out += String.fromCharCode(((code - 65 + s) % 26) + 65)
			continue
		}
		if (code >= 97 && code <= 122) {
			out += String.fromCharCode(((code - 97 + s) % 26) + 97)
			continue
		}
		out += ch
	}
	return out
}

function js_file_exists(relativePath) {
	const absolute = safeBenchmarkPath(relativePath)
	if (!fs.existsSync(absolute)) return false
	const stat = fs.statSync(absolute)
	if (stat.isDirectory()) return "folder"
	if (stat.isFile()) return "file"
	return false
}

function js_file_read(relativePath) {
	const absolute = safeBenchmarkPath(relativePath)
	const stat = fs.statSync(absolute)
	if (stat.isDirectory()) {
		return fs.readdirSync(absolute).sort()
	}
	return fs.readFileSync(absolute, "utf8")
}

function js_file_create(relativePath, content) {
	const absolute = safeBenchmarkPath(relativePath)
	if (content == null) {
		fs.mkdirSync(absolute, { recursive: true })
		return true
	}

	fs.mkdirSync(path.dirname(absolute), { recursive: true })
	fs.writeFileSync(absolute, content, "utf8")
	return true
}

function js_file_remove(relativePath) {
	const absolute = safeBenchmarkPath(relativePath)
	if (!fs.existsSync(absolute)) return false
	const stat = fs.statSync(absolute)
	if (stat.isDirectory()) {
		fs.rmSync(absolute, { recursive: true, force: false })
		return true
	}

	fs.unlinkSync(absolute)
	return true
}

function warmup(fn) {
	let sink = 0
	for (let i = 0; i < WARMUP_ITERS; i++) {
		sink += consume(fn(i))
	}
	return sink
}

function runCase(name, iterations, rustFn, jsFn) {
	warmup(rustFn)
	warmup(jsFn)

	let rustSink = 0
	profile.Start(`${name} | Abrupt: Rust`)
	for (let i = 0; i < iterations; i++) {
		rustSink += consume(rustFn(i))
	}
	const rustMs = profile.Stop()

	let jsSink = 0
	profile.Start(`${name} | Native: Javascript`)
	for (let i = 0; i < iterations; i++) {
		jsSink += consume(jsFn(i))
	}
	const jsMs = profile.Stop()

	const rustOps = ((iterations / rustMs) * 1000).toFixed(0)
	const jsOps = ((iterations / jsMs) * 1000).toFixed(0)

	console.log(`${name} sink check: ${rustSink | 0} / ${jsSink | 0}`)
	console.log(`${name} ops/sec: rust=${rustOps} js=${jsOps}`)
	console.log(`${name} ratio (Rust/JS): ${(rustMs / jsMs).toFixed(3)}x`)
	console.log("======================================")
}

runCase(
	"rand.int",
	FAST_ITERS,
	(i) => rand.int(i % 500, 1000),
	(i) => js_int(i % 500, 1000),
)
runCase(
	"rand.float",
	FAST_ITERS,
	(i) => rand.float(i % 500, 1000),
	(i) => js_float(i % 500, 1000),
)
runCase(
	"rand.str",
	MEDIUM_ITERS,
	(i) => rand.str(8 + (i % 16), true),
	(i) => js_str(8 + (i % 16), true),
)
runCase(
	"rand.bool",
	FAST_ITERS,
	() => rand.bool(),
	() => js_bool(),
)
runCase(
	"rand.array",
	FAST_ITERS,
	() => rand.array(SAMPLE_ARRAY),
	() => js_array(SAMPLE_ARRAY),
)
runCase(
	"rand.object(value)",
	MEDIUM_ITERS,
	() => rand.object(SAMPLE_OBJECT),
	() => js_object(SAMPLE_OBJECT),
)
runCase(
	"rand.object(value+keys)",
	MEDIUM_ITERS,
	() => rand.object(SAMPLE_OBJECT, SAMPLE_OBJECT_KEYS),
	() => js_object_with_keys(SAMPLE_OBJECT, SAMPLE_OBJECT_KEYS),
)
runCase(
	"rand.object(key+keys)",
	MEDIUM_ITERS,
	() => rand.object(SAMPLE_OBJECT, SAMPLE_OBJECT_KEYS, true),
	() => js_object_with_keys(SAMPLE_OBJECT, SAMPLE_OBJECT_KEYS, true),
)

const BASE64_PAYLOAD = js_base64_encode(BASE64_INPUT)
runCase(
	"crypto.base64.encode",
	HEAVY_ITERS,
	() => base64.encode(BASE64_INPUT),
	() => js_base64_encode(BASE64_INPUT),
)
runCase(
	"crypto.base64.decode",
	HEAVY_ITERS,
	() => base64.decode(BASE64_PAYLOAD),
	() => js_base64_decode(BASE64_PAYLOAD),
)
runCase(
	"crypto.sha256(hex)",
	HEAVY_ITERS,
	() => abruptCrypto.sha256(HASH_INPUT),
	() => js_sha256(HASH_INPUT, "hex"),
)
runCase(
	"crypto.sha256(base64)",
	HEAVY_ITERS,
	() => abruptCrypto.sha256(HASH_INPUT, "base64"),
	() => js_sha256(HASH_INPUT, "base64"),
)
runCase(
	"crypto.rot(13)",
	HEAVY_ITERS,
	() => abruptCrypto.rot(ROT_INPUT),
	() => js_rot(ROT_INPUT, 13),
)

resetBenchSandbox()
js_file_create(BENCH_FILE_REL, BASE64_INPUT)
js_file_create(`${BENCH_DIR_REL}/a.txt`, "a")
js_file_create(`${BENCH_DIR_REL}/b.txt`, "b")

runCase(
	"file.exists(file)",
	FILE_ITERS,
	() => file.exists(BENCH_FILE_REL),
	() => js_file_exists(BENCH_FILE_REL),
)
runCase(
	"file.read(file)",
	FILE_ITERS,
	() => file.read(BENCH_FILE_REL),
	() => js_file_read(BENCH_FILE_REL),
)
runCase(
	"file.read(folder)",
	FILE_ITERS,
	() => file.read(BENCH_DIR_REL),
	() => js_file_read(BENCH_DIR_REL),
)
runCase(
	"file.create+remove(file)",
	FILE_ITERS,
	(i) => {
		const relativePath = `${BENCH_ROOT_REL}/rust-cycle-${i & 255}.txt`
		file.create(relativePath, HASH_INPUT)
		return file.remove(relativePath)
	},
	(i) => {
		const relativePath = `${BENCH_ROOT_REL}/js-cycle-${i & 255}.txt`
		js_file_create(relativePath, HASH_INPUT)
		return js_file_remove(relativePath)
	},
)

cleanupRunArtifacts()
