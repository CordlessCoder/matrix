import com.googlecode.lanterna.TextColor;
import com.googlecode.lanterna.input.KeyStroke;
import com.googlecode.lanterna.screen.*;
import com.googlecode.lanterna.terminal.*;

import java.io.IOException;
import java.util.Random;

public class Main {
    public static void main(final String[] args) {
        final var terminalFactory = new DefaultTerminalFactory();
        try (var terminal = terminalFactory.createTerminal()) {
            try (var screen = new TerminalScreen(terminal)) {
                screen.startScreen();
                screen.setCursorPosition(null);

                final Random random = new Random();
                final var matrix = new Matrix(screen);

                final var color = new TextColor.RGB(0, 255, 0);

                boolean running = true;
                while (running) {
                    final long startTime = System.currentTimeMillis();
                    screen.doResizeIfNecessary();
                    screen.clear();
                    matrix.removeOffScreen();
                    matrix.draw(color);
                    matrix.advance();
                    screen.refresh(Screen.RefreshType.COMPLETE);
                    final int width = screen.getTerminalSize().getColumns();
                    final int lines = random.nextInt(1, width / 30 + 1);
                    for (int i = 0; i < lines; i++) {
                        matrix.addLine(random);
                    }
                    while (true) {
                        final long left = startTime + (1000 / 30) - System.currentTimeMillis();
                        if (left <= 0) {
                            break;
                        }
                        final KeyStroke input = screen.pollInput();
                        if (input != null) {
                            final Character c = input.getCharacter();
                            if (Character.toLowerCase(c) == 'q') {
                                running = false;
                                break;
                            }
                            continue;
                        }
                        try {
                            Thread.sleep(left);
                        } catch (final InterruptedException ignore) {
                            break;
                        }
                    }
                }
                screen.stopScreen();
            }
        } catch (final IOException e) {
            e.printStackTrace();
        }
    }
}
