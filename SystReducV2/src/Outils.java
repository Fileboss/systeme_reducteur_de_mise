
import java.util.ArrayList;
import java.util.List;

public class Outils {

	public static void testerPLusieursGrilles(List<Grille> ensemble, List<Grille> aTester, TableauBooleen tabBool,
			int garantie) {
		for (Grille g : aTester) {
			g.testerSansAjout(ensemble, tabBool, garantie);
			float pct = ((float) aTester.indexOf(g) / (float) aTester.size() * 100);
			System.out.println(pct);
		}
	}

	public static void optimiserPasOuf(List<Grille> bonnesGrilles, List<Grille> ensemble, int garantie) {

		List<Grille> bonnesGrillesSave = new ArrayList<>(bonnesGrilles);

		for (Grille g : bonnesGrillesSave) {
			TableauBooleen tabBool = new TableauBooleen(ensemble.size());
			bonnesGrilles.remove(g);
			Outils.testerPLusieursGrilles(ensemble, bonnesGrilles, tabBool, garantie);
			if (!tabBool.estEntièrementVrai()) {
				bonnesGrilles.add(g);
			}
		}

	}

	public static void convertir(List<Integer> systemeReduc, List<Grille> bonnesGrilles) {

		for (Grille g : bonnesGrilles) {
			g.remplacer(systemeReduc);
		}

	}
}
