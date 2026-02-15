import type {
	CryptoNamespace,
	ExportSurface,
	FileNamespace,
	RandNamespace,
} from "./types"

export * from "./types"

export declare const rand: RandNamespace
export declare const crypto: CryptoNamespace
export declare const file: FileNamespace

declare const abrupt: ExportSurface
export default abrupt
