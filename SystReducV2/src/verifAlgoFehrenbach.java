import java.util.ArrayList;
import java.util.List;

public class verifAlgoFehrenbach {

	public static void main(String[] args) {
		List<Grille> e = new ArrayList<>();
		e.add(new Grille(new GrilleBinaire(31)));
		e.add(new Grille(new GrilleBinaire(992)));
		e.add(new Grille(new GrilleBinaire(31744)));
		e.add(new Grille(new GrilleBinaire(1015808)));
		e.add(new Grille(new GrilleBinaire(533040)));
		e.add(new Grille(new GrilleBinaire(51268)));
		e.add(new Grille(new GrilleBinaire(67208)));
		e.add(new Grille(new GrilleBinaire(147498)));
		e.add(new Grille(new GrilleBinaire(139457)));
		e.add(new Grille(new GrilleBinaire(69906)));
		e.add(new Grille(new GrilleBinaire(264961)));
		e.add(new Grille(new GrilleBinaire(790660)));
		e.add(new Grille(new GrilleBinaire(37921)));
		e.add(new Grille(new GrilleBinaire(525634)));
		e.add(new Grille(new GrilleBinaire(303368)));
		e.add(new Grille(new GrilleBinaire(410640)));
		e.add(new Grille(new GrilleBinaire(34962)));
		e.add(new Grille(new GrilleBinaire(75812)));
		e.add(new Grille(new GrilleBinaire(530504)));
		e.add(new Grille(new GrilleBinaire(135940)));
		e.add(new Grille(new GrilleBinaire(606721)));
		e.add(new Grille(new GrilleBinaire(327792)));
		e.add(new Grille(new GrilleBinaire(263686)));
		e.add(new Grille(new GrilleBinaire(115072)));
		e.add(new Grille(new GrilleBinaire(172546)));
		e.add(new Grille(new GrilleBinaire(134408)));
		e.add(new Grille(new GrilleBinaire(20547)));
		e.add(new Grille(new GrilleBinaire(24724)));
		e.add(new Grille(new GrilleBinaire(657568)));
		e.add(new Grille(new GrilleBinaire(33368)));
		e.add(new Grille(new GrilleBinaire(77833)));
		e.add(new Grille(new GrilleBinaire(524581)));
		e.add(new Grille(new GrilleBinaire(197700)));
		e.add(new Grille(new GrilleBinaire(283168)));
		e.add(new Grille(new GrilleBinaire(525457)));
		e.add(new Grille(new GrilleBinaire(796674)));
		e.add(new Grille(new GrilleBinaire(574472)));
		e.add(new Grille(new GrilleBinaire(198673)));
		e.add(new Grille(new GrilleBinaire(262346)));
		e.add(new Grille(new GrilleBinaire(32940)));
		e.add(new Grille(new GrilleBinaire(10576)));
		e.add(new Grille(new GrilleBinaire(104960)));
		e.add(new Grille(new GrilleBinaire(9378)));
		e.add(new Grille(new GrilleBinaire(135320)));
		e.add(new Grille(new GrilleBinaire(680192)));
		e.add(new Grille(new GrilleBinaire(34068)));
		e.add(new Grille(new GrilleBinaire(344076)));
		e.add(new Grille(new GrilleBinaire(18569)));

		System.out.println(e);
		
		List<Grille> tot = Grille.enumererGrilles(20);
		TableauBooleen tabool = new TableauBooleen(tot.size());
		
		Outils.testerPLusieursGrilles(tot, e, tabool, 3);
		
		System.out.println("Pourcentage couvert : "+tabool.pourcentageVrai()+"%");

	}

}
