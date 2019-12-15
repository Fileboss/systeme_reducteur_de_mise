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

	public static void main(String[] args) {
		
		for (int garantis = 2; garantis < 5; garantis++) {
			for (int tailleSysReduc = 11; tailleSysReduc < 30; tailleSysReduc++) {
				if (garantis == 4 && tailleSysReduc == 26) break;
				int nbIterations = 0;
				List<Grille> grillesAJouerRetenues = Outils.genererCouverture(tailleSysReduc, garantis);
				for (int i = 0; i < nbIterations; i++) {
					List<Grille> grillesAJouerActuelles = Outils.genererCouverture(tailleSysReduc, garantis);
					if (grillesAJouerActuelles.size() < grillesAJouerRetenues.size()) {
						grillesAJouerRetenues = new ArrayList<>(grillesAJouerActuelles);
					}
//					double pct = Math.round(((double) i / nbIterations) * 100.0);
//					if (i % (nbIterations / 100) == 0)
//						System.out.println("Progression : " + pct + "%");
				}
				System.out.println("Pramètres : "+tailleSysReduc+" "+garantis);
				System.out.println("Taille de l'ensemble de grille retenu : " + grillesAJouerRetenues.size());
				String nFic = "Ressources/data_";
				nFic += tailleSysReduc + "_" + garantis+".txt";
				System.out.println("Nom du fichier : " + nFic);
				
				List<String> strGrilles = new ArrayList<>();
				for (Grille g : grillesAJouerRetenues) {
					strGrilles.add(g.toString());
				}
				
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
	}
	
	//		int tailleSysReduc = 20;
	//		int garantis = 2;

}
