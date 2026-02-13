import assert from "node:assert/strict"
import crypto from "node:crypto"
import fs from "node:fs"
import path from "node:path"
import test, { after } from "node:test"

import abrupt from "../index.mjs"

const { rand, crypto: abruptCrypto, file } = abrupt
const { base64 } = abruptCrypto

const REPO_ROOT = path.resolve(process.cwd())
const SANDBOX_REL = ".tmp/unit-test-sandbox"
const SANDBOX_ABS = path.resolve(REPO_ROOT, SANDBOX_REL)
const BENCH_SANDBOX_ABS = path.resolve(REPO_ROOT, "benchmark/.sandbox")
const TMP_ROOT_ABS = path.resolve(REPO_ROOT, ".tmp")
const BENCH_ROOT_ABS = path.resolve(REPO_ROOT, "benchmark")

function isWithin(base, target) {
	const basePath = path.resolve(base)
	const targetPath = path.resolve(target)
	if (basePath === targetPath) return true
	return targetPath.startsWith(`${basePath}${path.sep}`)
}

function ensureInsideRepo(targetPath) {
	assert.equal(
		isWithin(REPO_ROOT, targetPath),
		true,
		`Path escaped repository root: ${targetPath}`,
	)
}

function sandboxPath(...segments) {
	const relativePath = path.join(SANDBOX_REL, ...segments)
	const absolutePath = path.resolve(REPO_ROOT, relativePath)
	ensureInsideRepo(absolutePath)
	assert.equal(
		isWithin(SANDBOX_ABS, absolutePath),
		true,
		`Path escaped test sandbox: ${absolutePath}`,
	)
	return { relativePath, absolutePath }
}

function resetSandbox() {
	ensureInsideRepo(SANDBOX_ABS)
	fs.rmSync(SANDBOX_ABS, { recursive: true, force: true })
	fs.mkdirSync(SANDBOX_ABS, { recursive: true })
}

function cleanupSandboxes() {
	ensureInsideRepo(SANDBOX_ABS)
	fs.rmSync(SANDBOX_ABS, { recursive: true, force: true })
	ensureInsideRepo(BENCH_SANDBOX_ABS)
	fs.rmSync(BENCH_SANDBOX_ABS, { recursive: true, force: true })
	ensureInsideRepo(TMP_ROOT_ABS)
	fs.rmSync(TMP_ROOT_ABS, { recursive: true, force: true })
	ensureInsideRepo(BENCH_ROOT_ABS)
	fs.rmSync(BENCH_ROOT_ABS, { recursive: true, force: true })
}

after(() => {
	cleanupSandboxes()
})

test("exports are available", () => {
	assert.equal(typeof rand.int, "function")
	assert.equal(typeof rand.float, "function")
	assert.equal(typeof rand.str, "function")
	assert.equal(typeof rand.bool, "function")
	assert.equal(typeof rand.array, "function")
	assert.equal(typeof rand.object, "function")

	assert.equal(typeof abruptCrypto.md5, "function")
	assert.equal(typeof abruptCrypto.sha256, "function")
	assert.equal(typeof abruptCrypto.sha512, "function")
	assert.equal(typeof abruptCrypto.rot, "function")
	assert.equal(typeof base64.encode, "function")
	assert.equal(typeof base64.decode, "function")
	assert.equal(typeof abruptCrypto.base16.encode, "function")
	assert.equal(typeof abruptCrypto.base16.decode, "function")
	assert.equal(typeof abruptCrypto.base32.encode, "function")
	assert.equal(typeof abruptCrypto.base32.decode, "function")
	assert.equal(typeof abruptCrypto.AES.encode, "function")
	assert.equal(typeof abruptCrypto.AES.decode, "function")
	assert.equal(typeof abruptCrypto.RSA.encode, "function")
	assert.equal(typeof abruptCrypto.RSA.decode, "function")
	assert.equal(typeof abruptCrypto.morse.encode, "function")
	assert.equal(typeof abruptCrypto.morse.decode, "function")

	assert.equal(typeof file.create, "function")
	assert.equal(typeof file.exists, "function")
	assert.equal(typeof file.read, "function")
	assert.equal(typeof file.remove, "function")
})

test("rand namespace behaviors", () => {
	for (let i = 0; i < 200; i++) {
		const intValue = rand.int(20, 30)
		assert.equal(Number.isInteger(intValue), true)
		assert.equal(intValue >= 20 && intValue <= 30, true)
	}

	for (let i = 0; i < 200; i++) {
		const floatValue = rand.float(1.5, 3.5)
		assert.equal(floatValue >= 1.5 && floatValue <= 3.5, true)
	}

	const letters = rand.str(32, true)
	assert.match(letters, /^[a-z]{32}$/)

	const anyChars = rand.str()
	assert.equal(anyChars.length, 5)

	assert.equal(typeof rand.bool(), "boolean")

	const arraySample = [1, 2, 3, 4]
	for (let i = 0; i < 50; i++) {
		assert.equal(arraySample.includes(rand.array(arraySample)), true)
	}

	const objectSample = { a: 1, b: 2, c: 3 }
	const objectValues = Object.values(objectSample)
	const objectKeys = Object.keys(objectSample)
	for (let i = 0; i < 50; i++) {
		assert.equal(objectValues.includes(rand.object(objectSample)), true)
		assert.equal(objectKeys.includes(rand.object(objectSample, true)), true)
	}
})

test("crypto namespace behaviors", () => {
	const input = "Hello World!"
	const { base16, base32, AES, RSA, morse } = abruptCrypto

	const encoded = base64.encode(input)
	assert.equal(encoded, "SGVsbG8gV29ybGQh")
	assert.equal(base64.decode(encoded), input)

	const expectedMd5Hex = crypto.createHash("md5").update(input).digest("hex")
	const expectedMd5Base64 = crypto.createHash("md5").update(input).digest("base64")
	const expectedHex = crypto.createHash("sha256").update(input).digest("hex")
	const expectedBase64 = crypto
		.createHash("sha256")
		.update(input)
		.digest("base64")
	const expectedSha512Hex = crypto
		.createHash("sha512")
		.update(input)
		.digest("hex")
	const expectedSha512Base64 = crypto
		.createHash("sha512")
		.update(input)
		.digest("base64")
	assert.equal(abruptCrypto.md5(input), expectedMd5Hex)
	assert.equal(abruptCrypto.md5(input, "base64"), expectedMd5Base64)
	assert.equal(abruptCrypto.sha256(input), expectedHex)
	assert.equal(abruptCrypto.sha256(input, "base64"), expectedBase64)
	assert.equal(abruptCrypto.sha512(input), expectedSha512Hex)
	assert.equal(abruptCrypto.sha512(input, "base64"), expectedSha512Base64)

	assert.equal(abruptCrypto.rot(input), "Uryyb Jbeyq!")
	assert.equal(abruptCrypto.rot(input, 10), "Rovvy Gybvn!")

	const base16Encoded = base16.encode(input)
	assert.equal(base16Encoded, "48656c6c6f20576f726c6421")
	assert.equal(base16.decode(base16Encoded), input)

	const base32Encoded = base32.encode(input)
	assert.match(base32Encoded, /^[A-Z2-7]+=*$/)
	assert.equal(base32.decode(base32Encoded), input)

	const aesBase64 = AES.encode(input, "key")
	assert.equal(Array.isArray(aesBase64), true)
	assert.equal(aesBase64.length, 2)
	assert.equal(AES.decode(...aesBase64, "key"), input)

	const aesHex = AES.encode(input, "key", "hex")
	assert.equal(Array.isArray(aesHex), true)
	assert.equal(aesHex.length, 2)
	assert.equal(AES.decode(...aesHex, "key"), input)
	assert.equal(AES.decode(...aesHex, "key", "hex"), input)

	const rsaPacket = RSA.encode(input, "base64", 2048)
	assert.equal(typeof rsaPacket.encoded, "string")
	assert.equal(typeof rsaPacket.privateKey, "string")
	assert.equal(typeof rsaPacket.publicKey, "string")
	assert.equal(rsaPacket.encoding, "base64")
	assert.equal(rsaPacket.bits, 2048)
	assert.equal(RSA.decode(rsaPacket), input)

	const morseEncoded = morse.encode(input)
	assert.equal(typeof morseEncoded, "string")
	assert.equal(morse.decode(morseEncoded), "hello world!")
})

test("file namespace create/exists/read/remove", () => {
	resetSandbox()

	const helloFile = sandboxPath("Hello.txt")
	const hiFile = sandboxPath("Hi.txt")
	const welcomeFile = sandboxPath("Welcome.txt")
	const firstFolder = sandboxPath("Hello")
	const secondFolder = sandboxPath("World")
	const nestedFile = sandboxPath("this", "is", "three", "folders.txt")

	assert.equal(file.create(helloFile.relativePath, "Hello World!"), true)
	assert.equal(file.read(helloFile.relativePath), "Hello World!")

	assert.deepEqual(
		file.create(
			[hiFile.relativePath, welcomeFile.relativePath],
			["Hello", "World!"],
		),
		[true, true],
	)
	assert.equal(file.read(hiFile.relativePath), "Hello")
	assert.equal(file.read(welcomeFile.relativePath), "World!")

	assert.equal(file.create(firstFolder.relativePath), true)
	assert.deepEqual(
		file.create([firstFolder.relativePath, secondFolder.relativePath]),
		[true, true],
	)
	assert.deepEqual(
		file.create(firstFolder.relativePath, [secondFolder.relativePath]),
		[true, true],
	)
	assert.deepEqual(
		file.create(firstFolder.relativePath, secondFolder.relativePath),
		[true, true],
	)

	assert.equal(
		file.create(
			nestedFile.relativePath,
			"without this argument it would be a folder",
		),
		true,
	)
	assert.equal(
		file.read(nestedFile.relativePath),
		"without this argument it would be a folder",
	)

	assert.equal(file.exists(firstFolder.relativePath), "folder")
	assert.equal(file.exists(helloFile.relativePath), "file")
	assert.equal(file.exists(sandboxPath("not").relativePath), false)
	assert.deepEqual(
		file.exists([
			firstFolder.relativePath,
			helloFile.relativePath,
			sandboxPath("not").relativePath,
		]),
		["folder", "file", false],
	)
	assert.deepEqual(
		file.exists(
			firstFolder.relativePath,
			helloFile.relativePath,
			sandboxPath("not").relativePath,
		),
		["folder", "file", false],
	)
	assert.deepEqual(
		file.exists(firstFolder.relativePath, [
			helloFile.relativePath,
			sandboxPath("not").relativePath,
		]),
		["folder", "file", false],
	)

	const folderRead = file.read(firstFolder.relativePath)
	assert.equal(Array.isArray(folderRead), true)
	assert.equal(folderRead.includes(""), false)

	assert.equal(file.read(helloFile.relativePath, "utf8"), "Hello World!")
	assert.deepEqual(
		file.read([helloFile.relativePath, firstFolder.relativePath]),
		["Hello World!", folderRead],
	)
	assert.equal(file.read(sandboxPath("not").relativePath), false)

	assert.equal(file.remove(sandboxPath("this").relativePath), true)
	assert.deepEqual(
		file.remove([helloFile.relativePath, sandboxPath("not").relativePath]),
		[true, false],
	)
	assert.deepEqual(
		file.remove(
			hiFile.relativePath,
			welcomeFile.relativePath,
			firstFolder.relativePath,
		),
		[true, true, true],
	)
	assert.deepEqual(
		file.remove([sandboxPath("not").relativePath, secondFolder.relativePath]),
		[false, true],
	)
})

test("file namespace blocks unsafe paths", () => {
	resetSandbox()

	assert.throws(() => file.create("../outside.txt", "nope"))
	assert.throws(() => file.exists("/tmp"))
	assert.throws(() => file.read("../"))
	assert.throws(() => file.remove("."))
})

test("file namespace enforces deterministic argument contracts", () => {
	assert.throws(() => file.create())
	assert.throws(() => file.read())
	assert.throws(() => file.exists())
	assert.throws(() => file.remove())
	assert.throws(() => file.create("a", "b", "c"))
	assert.throws(() => file.read("a", "utf8", "extra"))
})
