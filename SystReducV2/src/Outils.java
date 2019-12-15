
import java.util.ArrayList;
import java.util.Collections;
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
	
	public static List<Grille> genererCouverture(int tailleSysReduc, int nbGarantis) {
		List<Grille> bonnesGrilles = new ArrayList<>();
		List<Grille> ensemble = Grille.enumererGrilles(tailleSysReduc);
		
		Collections.shuffle(ensemble);
		//int taille = ensemble.size();
		while(ensemble.size() > 0) {
			int i = 0;
			Grille gTemp = ensemble.get(0);
			ensemble.remove(0);
//			float pct = 100.0F - ((float) ensemble.size() / taille)*100;
//			System.out.println(pct + " ");	
			bonnesGrilles.add(gTemp);
			while (i < ensemble.size()) {
				if (gTemp.nbNombresEnCommun(ensemble.get(i)) >= nbGarantis) {
					ensemble.remove(i);
				} else {
					i++;
				}
			}
		}
		Collections.sort(bonnesGrilles);
		return bonnesGrilles;
	}
}
