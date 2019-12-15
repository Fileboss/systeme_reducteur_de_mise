import java.io.BufferedReader;
import java.io.IOException;
import java.nio.charset.Charset;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class RecupererLesData {
	
	public static void main(String[] args) {
		List<Integer> nbDuSysReduc = new ArrayList<>();
		
		//Ajout des nombres au sytème réducteur
			nbDuSysReduc.add(49);
			nbDuSysReduc.add(32);
			nbDuSysReduc.add(38);
			nbDuSysReduc.add(23);
			nbDuSysReduc.add(46);
			nbDuSysReduc.add(10);
			nbDuSysReduc.add(20);
			nbDuSysReduc.add(30);
			nbDuSysReduc.add(40);
			nbDuSysReduc.add(50);
			//10		
			nbDuSysReduc.add(21);
			nbDuSysReduc.add(7);
			nbDuSysReduc.add(1);
			nbDuSysReduc.add(28);
			nbDuSysReduc.add(17);
			nbDuSysReduc.add(16);
			nbDuSysReduc.add(2);
			nbDuSysReduc.add(48);
			nbDuSysReduc.add(12);
			nbDuSysReduc.add(14);
			//20
//			nbDuSysReduc.add(4);
//			nbDuSysReduc.add(36);
//			nbDuSysReduc.add(11);
//			nbDuSysReduc.add(41);
//			nbDuSysReduc.add(19);
//			nbDuSysReduc.add(13);
//			nbDuSysReduc.add(27);
//			nbDuSysReduc.add(42);
//			nbDuSysReduc.add(5);
//			nbDuSysReduc.add(25);
			//30
		
		final int TAILLESYSREDUC = nbDuSysReduc.size();
		final int GARANTIS = 3;
		
		String nFic = "Ressources/data_";
		nFic+=TAILLESYSREDUC+"_"+GARANTIS+".txt";
		Path path = Paths.get(nFic);
		BufferedReader monFic = null;;
		try {
			monFic = Files.newBufferedReader(path, Charset.forName("UTF-8"));
		} catch (IOException e) {
			e.printStackTrace();
		}
		
		
		List<Grille> grillesRecuperees = new ArrayList<>();
		String ligne = "";
		try {
			ligne = monFic.readLine();
		} catch (IOException e) {
			e.printStackTrace();
		}
		while (ligne != null) {
			grillesRecuperees.add(new Grille(ligne));
			try {
				ligne = monFic.readLine();
			} catch (IOException e) {
				e.printStackTrace();
			}
		}
		
		System.out.println( grillesRecuperees.size()+" grilles récupérées : \n"+grillesRecuperees);
		Outils.convertir(nbDuSysReduc, grillesRecuperees);
		System.out.println("\n"+grillesRecuperees.size()+" grilles récupérées converties : \n"+grillesRecuperees);
	}

}
