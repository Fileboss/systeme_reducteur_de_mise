
import java.io.BufferedReader;
import java.io.IOException;
import java.nio.charset.Charset;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Outils {

	public static void testerPLusieursGrilles(List<Grille> ensemble, List<Grille> aTester,
			TableauBooleen tabBool,
			int garantie) {
		for (Grille g : aTester) {
			g.testerSansAjout(ensemble, tabBool, garantie);
			float pct = ((float) aTester.indexOf(g) / (float) aTester.size() * 100);
			System.out.println(pct);
		}
	}

	public static void convertir(List<Integer> systemeReduc, List<Grille> bonnesGrilles) {

		for (Grille g : bonnesGrilles) {
			g.remplacer(systemeReduc);
		}

	}

	/**
	 * M�thode permettant de g�n�rer un ensemble de grille r�pondant au contraintes
	 * pass�es en param�tre
	 * 
	 * Entr�es :
	 * @param tailleSysReduc taille du sys�me r�ducteur
	 * @param nbGarantis     nombre de nombre garantis
	 * 
	 * Sortie :
	 * @return ensemble de grilles g�n�r�
	 * 
	 * Pr�Condition :
	 * @throws IllegalArgumentException quand tailleSysReduc < 11 ou tailleSysReduc
	 *                                  > 50 ou nbGarantis < 2 ou nbGarantis > 4
	 */

	public static List<Grille> genererCouverture(int tailleSysReduc, int nbGarantis)
			throws IllegalArgumentException {
		if (tailleSysReduc < 11 || tailleSysReduc > 50 || nbGarantis < 2
				|| nbGarantis > 4)
			throw new IllegalArgumentException("Argument invalide");

		// Ensemble de grilles que l'on va renvoyer
		List<Grille> bonnesGrilles = new ArrayList<>();

		// Ensemble de toutes les grilles possibles avec des entiers allant de 1 �
		// tailleSysReduc
		List<Grille> ensemble = Grille.enumererGrilles(tailleSysReduc);

		Collections.shuffle(ensemble);
		while (ensemble.size() > 0) {
			int i = 0;
			Grille gTemp = ensemble.get(0);
			ensemble.remove(0);
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

	public static List<String> recupererParametres(Float coefTaux, Float prix) {
		List<String> recups = new ArrayList<>();

		float coefInv = 1 - (coefTaux - 1);

		Path path = Paths.get("Ressources/prix.txt");
		BufferedReader monFic = null;
		;
		try {
			monFic = Files.newBufferedReader(path, Charset.forName("UTF-8"));
		} catch (IOException e) {
			System.out.println("Impossible d'ouvrir le fichier de prix");
			e.printStackTrace();
		}

		String ligne = "";
		try {
			ligne = monFic.readLine();
			if (Float.valueOf(ligne.substring(5)) < prix * coefTaux
					&& Float.valueOf(ligne.substring(5)) > prix * coefInv) {
				recups.add("Taille du syst�me r�ducteur : " + ligne.substring(0, 2)
						+ " Nombres garantis : " + ligne.charAt(3) + " Prix estim� : "
						+ ligne.substring(5) + " euros");
			}
		} catch (IOException e) {
			System.out.println("Erreur de lecture du fichier prix");
			e.printStackTrace();
		}

		while (ligne != null) {
			try {
				if (Float.valueOf(ligne.substring(5)) < prix * coefTaux
						&& Float.valueOf(ligne.substring(5)) > prix * coefInv) {
					recups.add("Taille du syst�me r�ducteur : " + ligne.substring(0, 2)
							+ " | Nombres garantis : " + ligne.charAt(3)
							+ " | Prix estim� : " + ligne.substring(5) + " euros");
				}
				ligne = monFic.readLine();
			} catch (IOException e) {
				System.out.println("Erreur de lecture du fichier prix");
				e.printStackTrace();
			}
		}

		return recups;
	}
}
