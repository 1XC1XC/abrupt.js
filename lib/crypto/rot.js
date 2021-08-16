let alpha = {}

for (let i = 97; i <= 122; i++) alpha[i-97] = String.fromCharCode(i)

module.exports = (input, amount = 13) => input.split(" ").map(x => x.split("").map(i => {
    let l = alpha[((i.toLowerCase().charCodeAt(0)-97)+amount)%26] || i
    if (i.toUpperCase() == i) l = l.toUpperCase()
    return l
})).map(x => x.join("")).join(" ")