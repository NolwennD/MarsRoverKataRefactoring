package org.codesmells.marsrover;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;

import org.junit.jupiter.api.Test;

class RoverTest {

  @Test
  void doTwo360() {
    var rover = new Rover(0,0,"N");
    
    var res = rover.execute("RRRRLLLL", List.of());
    
    assertThat(res).isEqualTo("0:0:N");
  }
  
  @Test
  void wrapAtSouthAndNorth() {
	  var rover = new Rover(0,0,"S");
	  
	  var res = rover.execute("MLLM", List.of());
	  
	  assertThat(res).isEqualTo("0:0:N");
  }
  
  @Test
  void wrapAtEastAndWest() {
	  var rover = new Rover(0,0,"W");
	  
	  var res = rover.execute("MLLM", List.of());
	  
	  assertThat(res).isEqualTo("0:0:E");
  }

  @Test
  void hitAnObstacleOnNorth() {
	  var rover = new Rover(0,0,"N");
	  
	  var res = rover.execute("M", List.of(List.of(0,1)));
	  
	  assertThat(res).isEqualTo("O:0:0:N");
  }

  @Test
  void hitAnObstacleOnSouth() {
	  var rover = new Rover(0,1,"S");
	  
	  var res = rover.execute("M", List.of(List.of(0,0)));
	  
	  assertThat(res).isEqualTo("O:0:1:S");
  }

  @Test
  void hitAnObstacleOnWest() {
	  var rover = new Rover(1,0,"W");
	  
	  var res = rover.execute("M", List.of(List.of(0,0)));
	  
	  assertThat(res).isEqualTo("O:1:0:W");
  }

  @Test
  void hitAnObstacleOnEast() {
	  var rover = new Rover(0,0,"E");
	  
	  var res = rover.execute("M", List.of(List.of(1,0)));
	  
	  assertThat(res).isEqualTo("O:0:0:E");
  }
  
}
