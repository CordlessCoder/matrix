import com.googlecode.lanterna.TextColor;
import com.googlecode.lanterna.screen.TerminalScreen;

import java.util.ArrayList;
import java.util.Random;

public class Matrix {
    private final ArrayList<Line> lines = new ArrayList<>();
    private final TerminalScreen screen;

    public Matrix(TerminalScreen screen) {
        this.screen = screen;
    }

    public void draw(TextColor.RGB color) {
        for (var line : lines) {
            line.draw(screen, color);
        }
    }

    public void advance() {
        for (var line : lines) {
            line.advance();
        }
    }

    public void addLine(Random rng) {
        var line = new Line(rng, (short) screen.getTerminalSize().getColumns());
        lines.add(line);
    }

    public void removeOffScreen() {
        int height = screen.getTerminalSize().getRows();
        lines.removeIf(l -> l.offScreen(height));
    }
}
