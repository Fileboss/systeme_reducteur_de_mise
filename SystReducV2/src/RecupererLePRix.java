import java.io.BufferedReader;
import java.io.IOException;
import java.nio.charset.Charset;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

public class RecupererLePRix {
	
	public static void main (String[] args) {
		int tailleSys = 25;
		int garantis = 4;
		
		String recup = (tailleSys+"_"+garantis);
		
		Path path = Paths.get("Ressources/prix.txt");
		BufferedReader monFic = null;
		try {
			monFic = Files.newBufferedReader(path, Charset.forName("UTF-8"));
		} catch (IOException e) {
			System.out.println("Erreur ouverture fichier");
			e.printStackTrace();
		}
		
		
		Float prixRecup = 0.0F;
		
		String ligne = "";
		try {
			ligne = monFic.readLine();
			if (recup.equals(ligne.substring(0, 4))) {
				prixRecup = Float.valueOf(ligne.substring(5));
			}
		} catch (IOException e) {
			System.out.println("Erreur ouverture fichier");
			e.printStackTrace();
		}
		
		while (ligne != null) {
			try {
				if (recup.equals(ligne.substring(0, 4))) {
					prixRecup = Float.valueOf(ligne.substring(5));
				}
				ligne = monFic.readLine();
			} catch (IOException e) {
				System.out.println("Erreur ouverture fichier");
				e.printStackTrace();
			}
		}
		
		System.out.println("Prix récupéré : \n"+prixRecup);
	}

}
