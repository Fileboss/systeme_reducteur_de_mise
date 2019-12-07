package deuxMilleDixHuit;

public class And extends OperandeBinaire {
	
	public And() {
		this.symbole = ".";
	}

	@Override
	public boolean evaluate() {
		return this.exps[0].evaluate() & this.exps[1].evaluate();
	}

}
