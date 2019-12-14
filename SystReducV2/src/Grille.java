
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Grille implements Comparable<Grille> {

	private List<Integer> nombres;

	public Grille(int a, int b, int c, int d, int e) {
		this.nombres = new ArrayList<>();

		this.nombres.add(a);
		this.nombres.add(b);
		this.nombres.add(c);
		this.nombres.add(d);
		this.nombres.add(e);

		Collections.sort(this.nombres);
	}

	public Grille(List<Integer> nombresAjout) {
		for (int e : nombresAjout) {
			this.nombres.add(e);
		}
		if (nombresAjout.size() < 5) {
			for (int i = 1; i <= 5 - nombresAjout.size(); i++) {
				this.nombres.add(i);
			}
		}
	}

	public List<Integer> getNombres() {
		return this.nombres;
	}

	@Override
	public String toString() {
		String res = "[";
		for (int i = 0; i < this.nombres.size() - 1; i++) {
			if (this.nombres.get(i) < 10)
				res += "0";
			res += this.nombres.get(i) + ", ";
		}
		if (this.nombres.get(this.nombres.size() - 1) < 10)
			res += "0";
		res += this.nombres.get(this.nombres.size() - 1) + "]";
		return res;
	}

	public static List<Grille> enumererGrilles(int nbNombres) {
		List<Grille> res = new ArrayList<>();
		int n = nbNombres;
		int r = 5;

		for (int i1 = 1; i1 < n - r + 1 + 1; i1++) {
			for (int i2 = i1 + 1; i2 < n - r + 2 + 1; i2++) {
				for (int i3 = i2 + 1; i3 < n - r + 3 + 1; i3++) {
					for (int i4 = i3 + 1; i4 < n - r + 4 + 1; i4++) {
						for (int i5 = i4 + 1; i5 < n - r + 5 + 1; i5++) {
							res.add(new Grille(i1, i2, i3, i4, i5));
						}
					}
				}
			}
		}
		return res;
	}

	public void testerAvecAjout(List<Grille> ensemble, TableauBooleen tabBool, int garantie,
			List<Grille> bonnesGrilles) {
		if (!tabBool.estEntièrementVrai()) {
			bonnesGrilles.add(this);
			for (int i = 0; i < ensemble.size(); i++) {
				if (this.nbNombresEnCommun(ensemble.get(i)) >= garantie) {
					tabBool.setVrai(i);
				}
			}
		}
	}

	public void testerSansAjout(List<Grille> ensemble, TableauBooleen tabBool, int garantie) {
		if (!tabBool.estEntièrementVrai()) {
			for (int i = 0; i < ensemble.size(); i++) {
				if (this.nbNombresEnCommun(ensemble.get(i)) >= garantie) {
					tabBool.setVrai(i);
				}
			}
		}
	}

	@Override
	public int compareTo(Grille o) {
		return this.toString().compareTo(o.toString());
	}

	@Override
	public int hashCode() {
		final int prime = 31;
		int result = 1;
		result = prime * result + ((nombres == null) ? 0 : nombres.hashCode());
		return result;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if (obj == null)
			return false;
		if (getClass() != obj.getClass())
			return false;
		Grille other = (Grille) obj;
		if (this.nombres == null) {
			if (other.nombres != null)
				return false;
		} else if (!this.toString().equals(other.toString()))
			return false;
		return true;
	}

	public List<Integer> nombresEnCommun(Grille g) {
		List<Integer> stock1 = new ArrayList<>(this.nombres);
		List<Integer> stock2 = new ArrayList<>(g.nombres);
		stock1.retainAll(stock2);
		return stock1;
	}

	public int nbNombresEnCommun(Grille g) {
		return this.nombresEnCommun(g).size();
	}

	public void remplacer(List<Integer> systemeReduc) {
		Collections.sort(systemeReduc);
		for (int i = 0; i < this.nombres.size(); i++) {
			this.nombres.set(i, systemeReduc.get(this.nombres.get(i) - 1));
		}

	}

}
