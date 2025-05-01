<?php
$a = 0;
$b = 1;
while (true) {
    usleep(1000);
    $a += $b;
    echo $a, "\n";
    $b += $a;
    echo $b, "\n";
}
?>