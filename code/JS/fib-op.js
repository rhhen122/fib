function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
var a=0;
var b=1;
while (true) {
    await sleep(1);
    a += b;
    console.log(a);
    b += a;
    console.log(b);
}
