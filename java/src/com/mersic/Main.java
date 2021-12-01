package com.mersic;

import java.nio.file.Files;
import java.nio.file.Path;

public class Main {
    
    public static void partOne() throws Exception {
        String filename = "./resources/day1.input";
        Integer[] lines = Files.readAllLines(Path.of(filename)).stream().map(x -> Integer.parseInt(x)).toArray(Integer[]::new);

        int incCount = 0;
        for (int i = 1; i < lines.length; i++) {
            if (lines[i-1] < lines[i]) {
                incCount++;
            }
        }

        System.out.println("partOne: " + incCount);
    }
    
    public static void partTwo() throws Exception {
        String filename = "./resources/day1.input";
        Integer[] lines = Files.readAllLines(Path.of(filename)).stream().map(x -> Integer.parseInt(x)).toArray(Integer[]::new);

        int incCount = 0;
        for (int i = 3; i < lines.length; i++) {
            int w1 = lines[i-3] + lines[i-2] + lines[i-1];
            int w2 = lines[i-2] + lines[i-1] + lines[i];
            if (w1 < w2) {
                incCount++;
            }
        }

        System.out.println("partTwo: " + incCount);
    }
    
    

    public static void main(String[] args) throws Exception {
        partOne();
        partTwo();
    }
}
