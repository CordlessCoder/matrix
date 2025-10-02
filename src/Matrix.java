import com.googlecode.lanterna.TextColor;
import com.googlecode.lanterna.screen.TerminalScreen;

import java.util.ArrayList;
import java.util.Random;

public class Matrix {
    private final ArrayList<Line> lines = new ArrayList<>();
    private final TerminalScreen screen;

    public Matrix(final TerminalScreen screen) {
        this.screen = screen;
    }

    public void draw(final TextColor.RGB color) {
        for (final var line : lines) {
            line.draw(screen, color);
        }
    }

    public void advance() {
        for (final var line : lines) {
            line.advance();
        }
    }

    public void addLine(final Random rng) {
        final var line = new Line(rng, (short) screen.getTerminalSize().getColumns());
        lines.add(line);
    }

    public void removeOffScreen() {
        final int height = screen.getTerminalSize().getRows();
        lines.removeIf(l -> l.offScreen(height));
    }
}
