<?php
declare(strict_types=1);

use MarsRover\MarsRover;
use PHPUnit\Framework\TestCase;

class MarsRoverTest extends TestCase
{

    public function testWithBadArguments(): void
    {
        $rover = new MarsRover(0, 0, "N");

        $result = $rover->execute("abcdEF", []);

        $this->assertEquals("0:0:N", $result);
    }

    public function testDoA360()
    {
        $rover = new MarsRover(0, 0, "N");

        $result = $rover->execute("RRRRLLLL", []);

        $this->assertEquals("0:0:N", $result);
    }

    public function testWrapNorthSouth()
    {
        $rover = new MarsRover(0, 0, "S");

        $result = $rover->execute("MLLM", []);

        $this->assertEquals("0:0:N", $result);
    }

    public function testWrapWestEast()
    {
        $rover = new MarsRover(0, 0, "W");

        $result = $rover->execute("MRRM", []);

        $this->assertEquals("0:0:E", $result);
    }

    public function testStuckInNorth()
    {
        $rover = new MarsRover(0,0, "N");

        $result = $rover->execute("M", [[0,1]]);

        $this->assertEquals("O:0:0:N", $result);
    }

    public function testStuckInSouth()
    {
        $rover = new MarsRover(0,0, "S");

        $result = $rover->execute("M", [[0,9]]);

        $this->assertEquals("O:0:0:S", $result);
    }

    public function testStuckInWest()
    {
        $rover = new MarsRover(0,0, "W");

        $result = $rover->execute("M", [[9,0]]);

        $this->assertEquals("O:0:0:W", $result);
    }

    public function testStuckInEast()
    {
        $rover = new MarsRover(0,0, "E");

        $result = $rover->execute("M", [[1,0]]);

        $this->assertEquals("O:0:0:E", $result);
    }
}
