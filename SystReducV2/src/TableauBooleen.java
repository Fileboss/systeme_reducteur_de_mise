
import java.util.ArrayList;
import java.util.List;

public class TableauBooleen {

	private List<Boolean> liste;

	public TableauBooleen(int taille) {
		this.liste = new ArrayList<>();
		for (int i = 0; i < taille; i++) {
			this.liste.add(false);
		}
	}

	public TableauBooleen(TableauBooleen tab) {
		this.liste = new ArrayList<>(tab.liste);
	}

	public void setVrai(int indice) {
		this.liste.set(indice, true);
	}

	public boolean estEntièrementVrai() {
		for (boolean b : this.liste) {
			if (!b)
				return false;
		}
		return true;
	}

	public int nbVrais() {
		int cpt = 0;
		for (boolean b : this.liste) {
			if (b)
				cpt++;
		}
		return cpt;
	}

	public float pourcentageVrai() {
		return ((float) this.nbVrais() / this.liste.size()) * 100;
	}

	@Override
	public String toString() {
		String res = "";
		for (int i = 0; i < this.liste.size(); i++) {
			res += i + " : " + this.liste.get(i) + "\n";
		}
		return res;
	}

}
