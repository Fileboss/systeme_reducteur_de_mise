package deuxMilleDixHuit;

public abstract class OperandeBinaire extends Operande {

	public OperandeBinaire() {
		this.exps = new ExpressionBooleenne[2];
	}
	
	public void setOperandeGauche(ExpressionBooleenne e) {
		this.exps[0] = e;
	}
	
	public void setOperandeDroite(ExpressionBooleenne e) {
		this.exps[1] = e;
	}

}
