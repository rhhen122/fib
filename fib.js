function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
var a=0;
var b=1;
var o=0;
while (true) {
    await sleep(1)
    console.log(a);
    o = a + b;
    a = b;
    b = o;
}