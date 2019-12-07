package deuxMilleDixHuit;

public class Implication extends OperandeBinaire {
	
	public Implication() {
		this.symbole = "=>";
	}

	@Override
	public boolean evaluate() {
		return !this.exps[0].evaluate() | this.exps[1].evaluate();
	}

}
