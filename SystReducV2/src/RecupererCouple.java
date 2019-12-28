import java.io.BufferedReader;
import java.io.IOException;
import java.nio.charset.Charset;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class RecupererCouple {

	public static void main(String[] args) {
		List<String> recups = new ArrayList<>();
		
		float prix = 150.0F;
		float coef = 1.15F;
		float coefInv = 1 - (coef-1);
		
		Path path = Paths.get("Ressources/prix.txt");
		BufferedReader monFic = null;;
		try {
			monFic = Files.newBufferedReader(path, Charset.forName("UTF-8"));
		} catch (IOException e) {
			System.out.println("Erreur ouverture fichier");
			e.printStackTrace();
		}
		
		String ligne = "";
		try {
			ligne = monFic.readLine();
			if (Float.valueOf(ligne.substring(5)) < prix*coef && Float.valueOf(ligne.substring(5)) > prix*coefInv) {
				recups.add("Taille du système réducteur : "+ligne.substring(0, 2)+" Nombres garantis : "+ligne.charAt(3));
			}
		} catch (IOException e) {
			System.out.println("Erreur ouverture fichier");
			e.printStackTrace();
		}
		
		while (ligne != null) {
			try {
				if (Float.valueOf(ligne.substring(5)) < prix*coef && Float.valueOf(ligne.substring(5)) > prix*coefInv) {
					recups.add("Taille du système réducteur : "+ligne.substring(0, 2)+" Nombres garantis : "+ligne.charAt(3));
				}
				ligne = monFic.readLine();
			} catch (IOException e) {
				System.out.println("Erreur ouverture fichier");
				e.printStackTrace();
			}
		}
		
		System.out.println("Paramètre récupérés :");
		System.out.println(recups);
	}
}
