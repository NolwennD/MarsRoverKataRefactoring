package org.codesmells.marsrover;

import java.util.List;

public final class Rover {

	private int x;
	private int y;
	private String heading;

	public Rover(int x, int y, String heading) {
		this.x = x;
		this.y = y;
		this.heading = heading;
	}

	public String execute(String commands, List<List<Integer>> obstacles) {
		boolean stuck = false;

		for (String command : commands.split("")) {
			if (command.equals("R") && heading.equals("N")) {
				heading = "E";
				continue;
			}
			if (command.equals("R") && heading.equals("E")) {
				heading = "S";
				continue;
			}
			if (command.equals("R") && heading.equals("S")) {
				heading = "W";
				continue;
			}
			if (command.equals("R") && heading.equals("W")) {
				heading = "N";
				continue;
			}
			if (command.equals("L") && heading.equals("N")){
				heading = "W";
				continue;
			}
			if (command.equals("L") && heading.equals("W")) {
				heading = "S";
				continue;
			}
			if (command.equals("L") && heading.equals("S")) {
				heading = "E";
				continue;
			}
			if (command.equals("L") && heading.equals("E")) {
				heading = "N";
				continue;
			}
			if (command.equals("M") && heading.equals("N")) {
				int next_y = y + 1;
				if (next_y > 9) {
					next_y = 0;
				}
				if (obstacles.contains(List.of(x, next_y))) {
					stuck = true;
				}
				if (!stuck) {
					y = next_y;
				}
			}
			if (command.equals("M") && heading.equals("S")) {
				int next_y = y - 1;
				if (next_y < 0) {
					next_y = 9;
				}
				if (obstacles.contains(List.of(x, next_y))) {
					stuck = true;
				}
				if (!stuck) {
					y = next_y;
				}
			}
			if (command.equals("M") && heading.equals("E")) {
				int next_y = x + 1;
				if (next_y > 9) {
					next_y = 0;
				}
				if (obstacles.contains(List.of(next_y, y))) {
					stuck = true;
				}
				if (!stuck) {
					x = next_y;
				}
			}
			if (command.equals("M") && heading.equals("W")) {
				int next_y = x - 1;
				if (next_y < 0) {
					next_y = 9;
				}

				if (obstacles.contains(List.of(next_y, y))) {
					stuck = true;
				}
				if (!stuck) {
					x = next_y;
				}
			}
		}

		return (stuck ? "O:" : "") + x + ":" + y + ":" + heading;
	}
}