"use strict"

const EXPORT_KEYS = Object.freeze(["rand", "crypto", "file"])

function requireArgs(name, args) {
	if (args.length > 0) return
	throw new TypeError(`${name} requires at least one argument`)
}

function normalizeVariadicArgs(name, args) {
	requireArgs(name, args)
	return args
}

function normalizeUnaryAndOptional(name, args) {
	requireArgs(name, args)
	if (args.length === 1) {
		return { inputs: args, option: undefined }
	}
	if (args.length === 2) {
		return { inputs: [args[0]], option: args[1] }
	}
	throw new TypeError(`${name} supports at most two arguments`)
}

function wrapFileNamespace(fileNs) {
	if (!fileNs) return fileNs

	return {
		create(...args) {
			const normalized = normalizeUnaryAndOptional("file.create", args)
			return fileNs.create(normalized.inputs, normalized.option)
		},
		exists(...args) {
			return fileNs.exists(normalizeVariadicArgs("file.exists", args))
		},
		read(...args) {
			const normalized = normalizeUnaryAndOptional("file.read", args)
			return fileNs.read(normalized.inputs, normalized.option)
		},
		remove(...args) {
			return fileNs.remove(normalizeVariadicArgs("file.remove", args))
		},
	}
}

function missingNamespaceError(name) {
	return new TypeError(`Native export "${name}" is missing`)
}

function getNativeNamespace(native, name) {
	const value = native[name]
	if (value) return value
	throw missingNamespaceError(name)
}

function wrapNative(native) {
	if (!native || typeof native !== "object") {
		throw new TypeError("Native module must be an object")
	}

	const rand = getNativeNamespace(native, EXPORT_KEYS[0])
	const crypto = getNativeNamespace(native, EXPORT_KEYS[1])
	const file = getNativeNamespace(native, EXPORT_KEYS[2])

	return {
		rand,
		crypto,
		file: wrapFileNamespace(file),
	}
}

module.exports = {
	EXPORT_KEYS,
	wrapNative,
}
