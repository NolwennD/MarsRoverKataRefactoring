using System;
using System.Collections.Generic;

namespace Codesmells.Marsrover
{
    public class Rover
    {
        private int _x;
        private int _y;
        private char _heading;

        public Rover(int x, int y, string heading)
        {
            this._x = x;
            this._y = y;
            this._heading = char.Parse(heading);
        }

        public string Execute(string commands, List<List<int>> obstacles)
        {
            bool stuck = false;

            foreach (var command in  commands.ToCharArray())
            {
                if (command.Equals('R') && _heading.Equals('N')) {
                    _heading = 'E';
                    continue;
                }
                if (command.Equals('R') && _heading.Equals('E')) {
                    _heading = 'S';
                    continue;
                }
                if (command.Equals('R') && _heading.Equals('S')) {
                    _heading = 'W';
                    continue;
                }
                if (command.Equals('R') && _heading.Equals('W')) {
                    _heading = 'N';
                    continue;
                }
                if (command.Equals('L') && _heading.Equals('N')){
                    _heading = 'W';
                    continue;
                }
                if (command.Equals('L') && _heading.Equals('W')) {
                    _heading = 'S';
                    continue;
                }
                if (command.Equals('L') && _heading.Equals('S')) {
                    _heading = 'E';
                    continue;
                }
                if (command.Equals('L') && _heading.Equals('E')) {
                    _heading = 'N';
                    continue;
                }
                if (command.Equals('M') && _heading.Equals('N')) {
                    int next_y = _y + 1;
                    if (next_y > 9) {
                        next_y = 0;
                    }
                    if (obstacles.FindIndex(element => element[0] == _x && element[1] == next_y) != -1) {
                        stuck = true;
                    }
                    if (!stuck) {
                        _y = next_y;
                    }
                }
                if (command.Equals('M') && _heading.Equals('S')) {
                    int next_y = _y - 1;
                    if (next_y < 0) {
                        next_y = 9;
                    }
                    if (obstacles.FindIndex(element => element[0] == _x && element[1] == next_y) != -1) {
                        stuck = true;
                    }
                    if (!stuck) {
                        _y = next_y;
                    }
                }
                if (command.Equals('M') && _heading.Equals('E')) {
                    int next_y = _x + 1;
                    if (next_y > 9) {
                        next_y = 0;
                    }
                    
                    if (obstacles.FindIndex(element => element[0] == next_y && element[1] == _y) != -1) {
                        stuck = true;
                    }
                    if (!stuck) {
                        _x = next_y;
                    }
                }
                if (command.Equals('M') && _heading.Equals('W')) {
                    int next_y = _x - 1;
                    if (next_y < 0) {
                        next_y = 9;
                    }
                    if (obstacles.FindIndex(element => element[0] == next_y && element[1] == _y) != -1) {
                        stuck = true;
                    }
                    if (!stuck) {
                        _x = next_y;
                    }
                }
            }

            return (stuck ? "O:" : "") + _x + ':' + _y + ':' + _heading;
        }
    }
}
