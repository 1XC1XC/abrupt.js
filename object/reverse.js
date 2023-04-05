module.exports = (x) => {
    let a = {}
    for (let i in x) a[x[i]] = i
    return a
}