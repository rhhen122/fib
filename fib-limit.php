<?php
$a = 0;
$b = 1;
$i = 0;
$limit = 10;
while (true) {
    usleep(1000);
    $a += $b;
    echo $a, "\n";
    $i += 1;
    $b += $a;
    echo $b, "\n";
    $i += 1;
    if ($i == $limit) {
        break;
    }
}
?>