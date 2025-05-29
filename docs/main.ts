const logo: string = "fib.png"; // This is the logo that will be used site wide
const name: string = "fib"; // This is the name that will be used site wide
const desc: string = "fib is a Fibonacci calculator written across multiple languages"; // This is the peice of text the appers at the top of the site and in embeds
const php = 0; // Change to one to load index.php (BETA)
const lice: string = "Apache"; /* Chose a license

 - CC
 - Apache
 - MIT
*/


var link = document.querySelector("link[rel~='icon']");
if (!link) {
    link = document.createElement('link');
    link.rel = 'icon';
    document.head.appendChild(link);
}
if (php == 1) {
  window.location.href = "./index.php";
}
if (lice == "CC") {
  let liceout = "https://badgen.net/static/license/CC 1.0/black?icon=https://www.svgrepo.com/show/362298/cc.svg";
} else if (lice == "Apache"){
  let liceout = "https://badgen.net/static/license/Apache%202.0/red?icon=https://upload.wikimedia.org/wikipedia/commons/7/7e/Apache_Feather_Logo.svg";
} else if (lice == "MIT") {
  let liceout = "https://badgen.net/static/license/MIT/blue?icon=https://www.svgrepo.com/show/105198/massachusetts-institute-of-technology-logotype.svg";
}
link.href = logo;
document.getElementById("lice").src = liceout;
document.getElementById("desctext").innerText = desc;
document.getElementById("desc").content = desc;
document.getElementById("title").innerText = name;
document.getElementById("logo").src = logo;
document.getElementById("name").innerText = name;
