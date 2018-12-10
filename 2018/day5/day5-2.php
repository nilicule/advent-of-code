<?php

// Get the polymer and keep a pristine copy
$polymer = trim(file_get_contents('day5.input'));
$untouchedPolymer = $polymer;

// Get all unique units
$unitTypes = array_unique(str_split(strtolower($polymer), 1));

foreach ($unitTypes as $unitType) {
    // Remove this unit type
    $polymer = removeUnit($untouchedPolymer, $unitType);

    // Get new polymer length
    $polymerLength = strlen($polymer);
    $unitIndex = 0;

    // React polymer
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

    // Add length to reaction results
    $reactionResults[$unitType] = $polymerLength;
}

echo "Shortest polymer has a length of " . min($reactionResults) . "\n";

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

function removeUnit(string $polymer, string $unit) : string {
    return str_ireplace($unit, '', $polymer);
}
