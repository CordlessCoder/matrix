import com.googlecode.lanterna.TextColor;
import com.googlecode.lanterna.input.KeyStroke;
import com.googlecode.lanterna.screen.*;
import com.googlecode.lanterna.terminal.*;

import javax.swing.*;
import java.io.IOException;
import java.util.Random;

public class Main {
    public static void main(String[] args) {
        var terminalFactory = new DefaultTerminalFactory();
        try (var terminal = terminalFactory.createTerminal()) {
            try (var screen = new TerminalScreen(terminal)) {
                screen.startScreen();
                screen.setCursorPosition(null);
                Random random = new Random();

                var matrix = new Matrix(screen);
                boolean running = true;
                var color = new TextColor.RGB(0, 255, 0);
                while (running) {
                    long startTime = System.currentTimeMillis();
                    screen.doResizeIfNecessary();
                    screen.clear();
                    matrix.removeOffScreen();
                    matrix.draw(color);
                    matrix.advance();
                    screen.refresh(Screen.RefreshType.DELTA);
                    int width = screen.getTerminalSize().getColumns();
                    int lines = random.nextInt(1, width / 30 + 1);
                    for (int i = 0; i < lines; i++) {
                        matrix.addLine(random);
                    }
                    while (true) {
                        long left = startTime + (1000 / 30) - System.currentTimeMillis();
                        if (left <= 0) {
                            break;
                        }
                        KeyStroke input = screen.pollInput();
                        if (input != null) {
                            Character c = input.getCharacter();
                            if (c != null && Character.toLowerCase(c) == 'q') {
                                running = false;
                                break;
                            }
                        }
                        try {
                            Thread.sleep(left);
                        } catch (InterruptedException ignore) {
                            break;
                        }
                    }
                }
                screen.stopScreen();
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}