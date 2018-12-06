<?php

$input = file_get_contents('day4.input');
$lines = explode(PHP_EOL, $input);

// Sort
sort($lines);

foreach ($lines as $guardLog) {
    // Parse 'begins shift'
    if (preg_match('/^\[(.*)\] Guard #(\d+) begins shift$/', $guardLog, $matches)) {
        $shiftStart = $matches[1];
        $shiftStartTimestamp = strtotime($shiftStart);
        $guard = $matches[2];
    }

    // Parse 'falls asleep'
    if (preg_match('/^\[(.*)\] falls asleep$/', $guardLog, $matches)) {
        $sleep = $matches[1];
        $sleepTimestamp = strtotime($sleep);
    }

    // Parse 'wakes up' and calculate minutes
    if (preg_match('/^\[(.*)\] wakes up$/', $guardLog, $matches)) {
        $awake = $matches[1];
        $awakeTimestamp = strtotime($awake);

        // Create new set if we haven't seen this guard before
        if (!isset($asleep[$guard])) {
            $asleep[$guard] = 0;
        }

        // Add seconds of sleeping
        $asleep[$guard] += abs($sleepTimestamp - $awakeTimestamp) / 60;

        // Determine minute most sleeping
        for ($timestamp = $sleepTimestamp; $timestamp < $awakeTimestamp; $timestamp += 60) {
            $minute = date('i', $timestamp);

            // Initialize array if new
            if (!isset($minutesSleeping[$guard][$minute])) {
                $minutesSleeping[$guard][$minute] = 0;
            }

            // Increase minute count
            $minutesSleeping[$guard][$minute]++;
        }
    }
}

foreach ($minutesSleeping as $guard => $sleepMinute) {
    // Get number of times most slept on minute
    $amountSleeping = max($sleepMinute);

    // Get the actual minute
    $minuteMostSlept = array_search($amountSleeping, $sleepMinute);

    $timesMostSpentSleeping[$guard] = $amountSleeping;
    $minutesMostSpentSleeping[$guard] = $minuteMostSlept;
}

$guardMostFrequentlySleeping = array_search(max($timesMostSpentSleeping), $timesMostSpentSleeping);
$minuteMostSpentSleeping = $minutesMostSpentSleeping[$guardMostFrequentlySleeping];

echo "Guard $guardMostFrequentlySleeping slept most at minute $minuteMostSpentSleeping, result ". $guardMostFrequentlySleeping * $minuteMostSpentSleeping ."\n";

//Guard 2081 slept most at minute 24, result 49944
