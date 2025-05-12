<?php
$a = 0;
$b = 1;
$o = 0;
while (true) {
    usleep(1000);
    echo $a, "\n";
    $o = $a + $b;
    $a = $b;
    $b = $o;
}
?>