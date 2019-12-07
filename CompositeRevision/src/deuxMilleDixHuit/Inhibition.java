package deuxMilleDixHuit;

public class Inhibition extends OperandeBinaire {
	
	public Inhibition() {
		this.symbole = "\\";
	}

	@Override
	public boolean evaluate() {
		return this.exps[0].evaluate() & !this.exps[1].evaluate();
	}

}
