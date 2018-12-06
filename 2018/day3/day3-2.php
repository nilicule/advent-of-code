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

        // Get all claimed squares for a claim
        $claims[] = [$claimId, squaresClaimed($inchesLeft, $inchesTop, $width, $height)];
    }
}

foreach ($claims as $claim) {
    list($claimId, $claimedSquares) = $claim;
    foreach ($claimedSquares as $claimedSquare) {
        $coordinate = getCoordinate($claimedSquare);
        if (array_key_exists($coordinate, $claimed)) {
            $overlap[$coordinate] = null;
        }
        $claimed[$coordinate] = null;
    }
}

foreach ($claims as $claim) {
    $overlapFound = false;
    list($claimId, $claimedSquares) = $claim;
    foreach ($claimedSquares as $claimedSquare) {
        $coordinate = getCoordinate($claimedSquare);
        if (array_key_exists($coordinate, $overlap)) {
            $overlapFound = true;
            continue;
        }
    }
    if (!$overlapFound) {
        echo "ClaimId without overlap: $claimId\n";
        break;
    }
}

function squaresClaimed(int $inchesLeft, int $inchesTop, int $width, int $height): array
{
    $claimedSquares = [];
    for ($y = $inchesTop; $y < $inchesTop + $height; $y += 1) {
        for ($x = $inchesLeft; $x < $inchesLeft + $width; $x += 1) {
            $claimedSquares[] = [$x, $y];
        }
    }

    return $claimedSquares;
}

function getCoordinate(array $claimedSquare): string
{
    list($x, $y) = $claimedSquare;

    return "$x,$y";
}

//ClaimId without overlap: 1270
