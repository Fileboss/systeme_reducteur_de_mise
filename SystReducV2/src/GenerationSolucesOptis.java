import java.io.File;
import java.io.IOException;
import java.nio.charset.Charset;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;
import java.util.ArrayList;
import java.util.List;

public class GenerationSolucesOptis {

	/**
	 * Maintenant g�n�re aussi le prix pour chaque set de grilles
	 */
	public static void main(String[] args) {
		
		//V�rification de la pr�sence du dossier Ressources et cr�ation le cas �ch�ant
		Path folder = Paths.get("Ressources");
		if (Files.notExists(folder)) {
			File folde = new File("Ressources");
		      boolean bool = folde.mkdir();
		      if(bool){
		         System.out.println("R�pertoire cr��e");
		      }else{
		         System.out.println("Impossible de cr�er le r�pertorie");
		      }
		}
		
		//Liste dans laquelle on va r�cup�rer les prix en fonction des param�tres
		List<String> strPrix = new ArrayList<>();
		
		for (int garantis = 2; garantis < 5; garantis++) {
			for (int tailleSysReduc = 11; tailleSysReduc < 31; tailleSysReduc++) {
				if (garantis == 4 && tailleSysReduc == 26) break;
				int nbIterations = 3;
				//G�n�ration des sets de grilles et conservation du meilleur pour chaque param�tres
				List<Grille> grillesAJouerRetenues = Outils.genererCouverture(tailleSysReduc, garantis);
				for (int i = 0; i < nbIterations; i++) {
					List<Grille> grillesAJouerActuelles = Outils.genererCouverture(tailleSysReduc, garantis);
					if (grillesAJouerActuelles.size() < grillesAJouerRetenues.size()) {
						grillesAJouerRetenues = new ArrayList<>(grillesAJouerActuelles);
					}
				}
				//Affichage
				System.out.println("\nPram�tres : "+tailleSysReduc+" "+garantis);
				System.out.println("Taille de l'ensemble de grilles retenu : " + grillesAJouerRetenues.size());
				
				//Cr�ation du nom du fichier en fontion des param�tres actuels
				String nFic = "Ressources/data_";
				nFic += tailleSysReduc + "_" + garantis+".txt";
				System.out.println("Nom du fichier : " + nFic);
				
				strPrix.add(tailleSysReduc + "_" + garantis+","+new Float(grillesAJouerRetenues.size()*2.5F).toString());
				
				//Transformation de la liste de grilles retenue en liste de String
				List<String> strGrilles = new ArrayList<>();
				for (Grille g : grillesAJouerRetenues) {
					strGrilles.add(g.toString());
				}
				
				//Ecriture des grilles dans le fichier
				File f = new File(nFic);
				try {
					f.createNewFile();
				} catch (IOException e1) {
					e1.printStackTrace();
				}
				Path fichier = Paths.get(nFic);		
				try {
					Files.write(fichier, strGrilles, Charset.forName("UTF-8"), StandardOpenOption.APPEND);
				} catch (IOException e) {
					e.printStackTrace();
				}
			}
		}
		//Ecriture des correspondances param�tres / prix dans le fichier prix.txt
		String ficPrix = "Ressources/prix.txt";
		File fic = new File(ficPrix);
		try {
			fic.createNewFile();
		} catch (IOException e1) {
			System.out.println("Erreur creation du fichier prix");
			e1.printStackTrace();
		}
		Path fichierPrix = Paths.get(ficPrix);
		try {
			Files.write(fichierPrix, strPrix, Charset.forName("UTF-8"), StandardOpenOption.APPEND);
		} catch (IOException e) {
			System.out.println("Erreur �criture dans le fichier prix");
			e.printStackTrace();
		}
		System.out.println("\n\nG�n�ration termin�e avec succ�s");
	}

}
