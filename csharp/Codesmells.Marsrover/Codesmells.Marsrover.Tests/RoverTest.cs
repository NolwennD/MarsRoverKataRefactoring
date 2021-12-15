using System.Collections.Generic;
using NUnit.Framework;

namespace Codesmells.Marsrover.Tests
{
    public class RoverTest
    {
        [SetUp]
        public void Setup()
        {
        }


        [Test]
        public void doTwo360() {
            var rover = new Rover(0,0,"N");
    
            var res = rover.Execute("RRRRLLLL", new List<List<int>>());
    
            Assert.AreEqual("0:0:N",res);
        }
  
        [Test]
        public void wrapAtSouthAndNorth() {
            var rover = new Rover(0,0,"S");
	  
            var res = rover.Execute("MLLM", new List<List<int>>());
	  
            Assert.AreEqual("0:0:N",res);
        }
  
        [Test]
        public void wrapAtEastAndWest() {
            var rover = new Rover(0,0,"W");
	  
            var res = rover.Execute("MLLM", new List<List<int>>());
	  
            Assert.AreEqual("0:0:E",res);
        }

        [Test]
        public void hitAnObstacleOnNorth() {
            var rover = new Rover(0,0,"N");
	  
            var res = rover.Execute("M", new List<List<int>>(){ new List<int>() {0,1}});
	  
            Assert.AreEqual("O:0:0:N",res);
        }

        [Test]
        public void hitAnObstacleOnSouth() {
            var rover = new Rover(0,1,"S");
	  
            var res = rover.Execute("M", new List<List<int>>(){ new List<int>() {0,0}});
	  
            Assert.AreEqual("O:0:1:S",res);
        }

        [Test]
        public void hitAnObstacleOnWest() {
            var rover = new Rover(1,0,"W");
	  
            var res = rover.Execute("M", new List<List<int>>(){ new List<int>() {0,0}});
	  
            Assert.AreEqual("O:1:0:W",res);
        }

        [Test]
        public void hitAnObstacleOnEast() {
            var rover = new Rover(0,0,"E");
	  
            var res = rover.Execute("M", new List<List<int>>(){ new List<int>() {1,0}});
	  
            Assert.AreEqual("O:0:0:E",res);
        }

    }
}