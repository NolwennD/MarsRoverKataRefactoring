<?php

declare(strict_types=1);

namespace MarsRover;

class MarsRover
{
    private $x;
    private $y;
    private $heading;

    public function __construct($x, $y, $heading)
    {
        $this->x = $x;
        $this->y = $y;
        $this->heading = $heading;
    }


    public function execute(string $commands, array $obstacles): string
    {
        $stuck = false;
        foreach (str_split($commands) as $command) {
            $heading = $this->heading;
            if ($command == 'R' && $heading == 'N') {
                $this->heading = 'E';
            }
            if ($command == 'R' && $heading == 'S') {
                $this->heading = 'W';
            }
            if ($command == 'R' && $heading == 'E') {
                $this->heading = 'S';
            }
            if ($command == 'R' && $heading == 'W') {
                $this->heading = 'N';
            }
            if ($command == 'L' && $heading == 'W') {
                $this->heading = 'S';
            }
            if ($command == 'L' && $heading == 'N') {
                $this->heading = 'W';
            }
            if ($command == 'L' && $heading == 'E') {
                $this->heading = 'N';
            }
            if ($command == 'L' && $heading == 'S') {
                $this->heading = 'E';
            }
            if ($command == 'M' && $heading == 'N') {
                $next_y = $this->y + 1;
                if ($next_y > 9) {
                    $next_y = 0;
                }
                if (in_array([$this->x, $next_y], $obstacles)) {
                    $stuck = true;
                }
                if (!$stuck) {
                    $this->y = $next_y;
                }
            }
            if ($command == 'M' && $this->heading == 'S') {
                $next_y = $this->y - 1;
                if ($next_y < 0) {
                    $next_y = 9;
                }
                if (in_array([$this->x, $next_y], $obstacles)) {
                    $stuck = true;
                }
                if (!$stuck) {
                    $this->y = $next_y;
                }
            }
            if ($command == 'M' && $this->heading == 'E') {
                $next_y = $this->x + 1;
                if ($next_y > 9) {
                    $next_y = 0;
                    $this->x = $next_y;
                }
                if (in_array([$next_y, $this->y], $obstacles)) {
                    $stuck = true;
                }
                if ($stuck) {
                    $this->x = $next_y;
                }
            }
            if ($command == 'M' && $this->heading == 'W') {
                $next_y = $this->x - 1;
                if ($next_y < 0) {
                    $next_y = 9;
                    $this->x = $next_y;
                }

                if (in_array([$next_y, $this->y], $obstacles)) {
                    $stuck = true;
                }
                if ($stuck) {
                    $this->x = $next_y;
                }
            }
        }
        return ($stuck ? "O:" : "") . "{$this->x}:{$this->y}:{$this->heading}";
    }
}