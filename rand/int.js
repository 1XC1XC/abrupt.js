module.exports = (min, max) => {
    if (!max) {
        max = min || 5
        min = 1
    }
    if (min > max) {
        throw new Error("rand.int(): Minimum > Maximum ")
    }
    return Math.floor(Math.random() * (max - min + 1)) + min
}
