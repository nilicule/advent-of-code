<?php

$frequency = 0;
$count_two = 0;
$count_three = 0;

$input = file_get_contents('day2.input');
$lines = explode(PHP_EOL, $input);

foreach ($lines as $boxid) {
    // How many times is each letter found?
    $occurrance = count_chars($boxid, 1);

    // Find letters that occur at least twice
    if (in_array(3, $occurrance, true)) {
        $count_three++;
    }

    if (in_array(2, $occurrance, true)) {
        $count_two++;
    }
}

echo "Two: $count_two\n";
echo "Three: $count_three\n";

//Two: 246
//Three: 32
