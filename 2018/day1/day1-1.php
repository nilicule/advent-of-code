<?php

$frequency = 0;

$input = file_get_contents('day1.input');
$lines = explode(PHP_EOL, $input);

foreach ($lines as $line) {
    $frequency = $frequency + (int) $line;
}

echo $frequency;

//470
