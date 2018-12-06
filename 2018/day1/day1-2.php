<?php

$frequency = 0;
$frequency_found = [0];
$loop_required = true;
$loops = 0;

//$input = file_get_contents('day1.input');
//$lines = explode(PHP_EOL, $input);

while ($loop_required) {
    echo "Loop: $loops\n";
    $handle = fopen("day1.input", "r");
    if ($handle) {
        while (($line = fgets($handle)) !== false) {
            $old_frequency = $frequency;
            $change = (int) $line;
            $frequency = $old_frequency + (int) $line;
            //echo "current: $old_frequency, change: $change, now: $frequency\n";

            if (in_array($frequency, $frequency_found, true)) {
                echo "Already seen $frequency [loops: $loops]!\n";
                exit;
            }

            $frequency_found[] = $frequency;
        }

        fclose($handle);
    }
    $loops++;
}

echo $frequency;

//Already seen 790 [loops: 137]!
