<?php

$match = [];

$input = file_get_contents('day2.input');
$lines = explode(PHP_EOL, $input);

foreach ($lines as $primary_boxid) {
    foreach ($lines as $secondary_boxid) {
        if (in_array($secondary_boxid, $match)) {
            continue;
        }

        if (levenshtein($primary_boxid, $secondary_boxid) === 1) {
            $match[] = $secondary_boxid;
        }
    }
}

if (count($match) === 2) {
    // find first char that is different using bitwise XOR. Similar chars will become null bytes ("\0")
    $position = strspn($match[0] ^ $match[1], "\0");
    echo substr_replace($match[0], '', $position, 1) . "\n";
} else {
    echo "No match found!\n";
}

// tjxmoewpdkyaihvrndfluwbzc
