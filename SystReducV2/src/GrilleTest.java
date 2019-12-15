import static org.junit.Assert.*;

import org.junit.Test;

public class GrilleTest {

	@Test
	public void test() {
		assertEquals(new Grille(1, 2, 3, 4, 5).toString(), new Grille("01,02,03,04,05").toString());
	}

}
