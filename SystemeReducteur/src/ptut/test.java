package ptut;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class test {

	public static void main(String[] args) {
		
		final int GARANTIE = 3;
		
		

		List<Grille> bonnesGrilles = new ArrayList<>();
		List<Integer> nbDuSysReduc = new ArrayList<>();
		
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
		
		Collections.sort(nbDuSysReduc);
		
		
		final int SYSREDUC = nbDuSysReduc.size();
		
		
		List<Grille> ensemble = Grille.enumererGrilles(SYSREDUC);
		System.out.println("taille de l'ensemble avant conversion : "+ensemble.size());
		//Outils.convertir(nbDuSysReduc, ensemble);
		System.out.println("taille de l'ensemble apr�s conversion : "+ensemble.size());
		//System.out.println(ensemble.toString());
		//System.out.println("Nb grilles g�n�r�es : "+ensembleSave.size());
		
		TableauBooleen taboule = new TableauBooleen(ensemble.size());
		//System.out.println("grille � tester : "+ensemble.get(0));
		///ensemble.get(0).tester(ensemble, taboule, GARANTIE, bonnesGrilles);
		//ensemble.get(ensemble.size()-1).tester(ensemble, taboule, GARANTIE, bonnesGrilles);
		
		List<Grille> ensembleSave = new ArrayList<>(ensemble);
		
		
		
		System.out.println("Poucentage du tableau avant algo : "+taboule.pourcentageVrai()+" nb bonnes grilles avant algo : "+bonnesGrilles.size());
		
		
		//Methode c�dric
		Collections.shuffle(ensemble);
		while(ensemble.size() > 0) {
			int i = 0;
			Grille gTemp = ensemble.get(0);
			ensemble.remove(0);
			float pct = 100.0F - ((float) ensemble.size() / ensembleSave.size())*100;
			System.out.println(pct + " ");
			bonnesGrilles.add(gTemp);
			while (i < ensemble.size()) {
				if (gTemp.nbNombresEnCommun(ensemble.get(i)) >= GARANTIE) {
					ensemble.remove(i);
				} else {
					i++;
				}
			}
		}
		//Jusqu'ici
		
		
		System.out.println("Fin de l'algo\nCalcul de la couverture...\n");
		
		Outils.testerPLusieursGrilles(ensembleSave, bonnesGrilles, taboule, GARANTIE);
		
		System.out.println("\nNb grilles g�n�r�es : "+ensembleSave.size());
		
		System.out.println("\nPourcentage couvert "+taboule.pourcentageVrai()+" nombre de grilles gard�es : "+bonnesGrilles.size());
		
		System.out.println("Systeme R�ducteur : ");
		System.out.println(nbDuSysReduc);
		
		System.out.println("\nGrilles gard�es non converties\n"+bonnesGrilles);
		Outils.convertir(nbDuSysReduc, bonnesGrilles);
		System.out.println("\n"+bonnesGrilles.size()+"  Grilles gard�es  converties\n"+bonnesGrilles);
		
	}

}
