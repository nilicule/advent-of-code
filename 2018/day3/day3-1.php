<?php

$input = file_get_contents('day3.input');
$lines = explode(PHP_EOL, $input);

$claims = [];
$claimed = [];
$overlap = [];

// Decode all claims
foreach ($lines as $line) {
    // Decode claim
    if (preg_match('/#(\d+) @ (\d+),(\d+): (\d+)x(\d+)/', $line, $matches)) {
        $claimId = $matches[1];
        $inchesLeft = $matches[2];
        $inchesTop = $matches[3];
        $width = $matches[4];
        $height = $matches[5];

        // Add claim to array
        $claims[] = [$claimId, $inchesLeft, $inchesTop, $width, $height];
    }
}

foreach ($claims as $claim) {
    list($claimID, $left, $top, $width, $height) = $claim;
    for ($y = $top; $y < $top + $height; $y++) {
        for ($x = $left; $x < $left + $width; $x++) {
            $coordinate = "$x,$y";
            if (array_key_exists($coordinate, $claimed)) {
                $overlap[$coordinate] = null;
            }
            $claimed[$coordinate] = null;
        }
    }
}

echo count($overlap) . PHP_EOL;

// 118539
