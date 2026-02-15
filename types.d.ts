export type BinaryEncoding = "hex" | "base64"
export type AesEncoding = "base64" | "hex"
export type PathInput = string | string[]
export type ExistsEntry = "file" | "folder" | false
export type ReadEntry = string | string[] | false
export type ExportKey = "rand" | "crypto" | "file"

export interface RsaPacket {
    encoded: string
    privateKey: string
    publicKey: string
    encoding: BinaryEncoding
    bits: number
}

export interface RandNamespace {
    int(min?: number, max?: number): number
    float(min?: number, max?: number): number
    str(lengthOrLetters?: number | boolean, letters?: boolean): string
    bool(): boolean
    array(values: readonly unknown[]): unknown | null
    object(
        values: Record<string, unknown>,
        keyOrKeys?: boolean | readonly string[],
        returnKey?: boolean,
    ): unknown | null
}

export interface BaseCodecNamespace {
    encode(input: string): string
    decode(input: string): string
}

export interface AesNamespace {
    encode(input: string, key: string, encoding?: AesEncoding): [encoded: string, ivHex: string]
    decode(encoded: string, ivHex: string, key: string, encoding?: AesEncoding): string
}

export interface RsaNamespace {
    encode(input: string, encoding?: BinaryEncoding, bits?: number): RsaPacket
    decode(packet: RsaPacket): string
}

export interface MorseNamespace {
    encode(input: string): string
    decode(input: string): string
}

export interface CryptoNamespace {
    base64: BaseCodecNamespace
    base16: BaseCodecNamespace
    base32: BaseCodecNamespace
    md5(input: string, encoding?: BinaryEncoding): string
    sha256(input: string, encoding?: BinaryEncoding): string
    sha512(input: string, encoding?: BinaryEncoding): string
    AES: AesNamespace
    RSA: RsaNamespace
    morse: MorseNamespace
    rot(input: string, shift?: number): string
}

export interface FileNamespace {
    create(pathInput: PathInput, contentOrPaths?: string | string[]): boolean | boolean[]
    exists(...inputs: PathInput[]): ExistsEntry | ExistsEntry[]
    read(pathInput: PathInput, encoding?: string): ReadEntry | ReadEntry[]
    remove(...inputs: PathInput[]): boolean | boolean[]
}

export interface ExportSurface {
    rand: RandNamespace
    crypto: CryptoNamespace
    file: FileNamespace
}

export type AbruptModule = ExportSurface
