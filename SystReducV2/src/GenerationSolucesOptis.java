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
	 * Maintenant génère aussi le prix pour chaque set de grilles
	 */
	public static void main(String[] args) {
		
		//Vérification de la présence du dossier Ressources et création le cas échéant
		Path folder = Paths.get("Ressources");
		if (Files.notExists(folder)) {
			File folde = new File("Ressources");
		      boolean bool = folde.mkdir();
		      if(bool){
		         System.out.println("Répertoire créée");
		      }else{
		         System.out.println("Impossible de créer le répertorie");
		      }
		}
		
		//Liste dans laquelle on va récupérer les prix en fonction des paramètres
		List<String> strPrix = new ArrayList<>();
		
		for (int garantis = 2; garantis < 5; garantis++) {
			for (int tailleSysReduc = 11; tailleSysReduc < 31; tailleSysReduc++) {
				if (garantis == 4 && tailleSysReduc == 26) break;
				int nbIterations = 3;
				//Génération des sets de grilles et conservation du meilleur pour chaque paramètres
				List<Grille> grillesAJouerRetenues = Outils.genererCouverture(tailleSysReduc, garantis);
				for (int i = 0; i < nbIterations; i++) {
					List<Grille> grillesAJouerActuelles = Outils.genererCouverture(tailleSysReduc, garantis);
					if (grillesAJouerActuelles.size() < grillesAJouerRetenues.size()) {
						grillesAJouerRetenues = new ArrayList<>(grillesAJouerActuelles);
					}
				}
				//Affichage
				System.out.println("\nPramètres : "+tailleSysReduc+" "+garantis);
				System.out.println("Taille de l'ensemble de grilles retenu : " + grillesAJouerRetenues.size());
				
				//Création du nom du fichier en fontion des paramètres actuels
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
		//Ecriture des correspondances paramètres / prix dans le fichier prix.txt
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
			System.out.println("Erreur écriture dans le fichier prix");
			e.printStackTrace();
		}
		System.out.println("\n\nGénération terminée avec succès");
	}

}
