package deuxMilleDixHuit;

public class Equivalence extends OperandeBinaire {
	
	public Equivalence() {
		this.symbole = "<=>";
	}

	@Override
	public boolean evaluate() {
		boolean a = this.exps[0].evaluate();
		boolean b = this.exps[1].evaluate();
		return (a&&b) || (!a&&!b);
	}

}
