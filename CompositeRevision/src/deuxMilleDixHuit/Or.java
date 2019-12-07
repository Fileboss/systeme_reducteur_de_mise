package deuxMilleDixHuit;

public class Or extends OperandeBinaire {
	
	public Or() {
		this.symbole = "+";
	}

	@Override
	public boolean evaluate() {
		return this.exps[0].evaluate() | this.exps[1].evaluate();
	}

}
