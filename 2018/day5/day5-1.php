<?php

$polymer = trim(file_get_contents('day5.input'));

$polymerLength = strlen($polymer);
$unitIndex = 0;

while ($polymerLength > 0 && $unitIndex < $polymerLength) {
    // Get a unit
    $unit = $polymer[$unitIndex];

    // get unit neighbor, take string length into account
    $nextUnitIndex = $unitIndex + 1;
    $nextUnit = $polymer[$nextUnitIndex] ?? '';

    // Check if unit reacts
    if (hasReaction($unit, $nextUnit)) {
        // Destroy units
        $polymer = destroyUnits($polymer, $unitIndex);

        // Set new polymer length
        $polymerLength = strlen($polymer);

        // Since two units were removed, take one step back
        $unitIndex = max(0, $unitIndex - 1);
        continue;
    }
    $unitIndex++;
}

echo strlen($polymer) . " units remain after fully reacting\n";

function hasReaction(string $a, string $b) : bool {
    // Units are not of the same type
    if (strtolower($a) !== strtolower($b)) {
        return false;
    }

    // Matching polarities do not react
    if ($a === $b) {
        return false;
    }

    // The rest reacts
    return true;
}

function destroyUnits(string $polymer, int $unitIndex) : string {
    return substr($polymer, 0, $unitIndex) . substr($polymer, $unitIndex + 2);
}
