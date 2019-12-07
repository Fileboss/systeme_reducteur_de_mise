package deuxMilleDixHuit;

public abstract class OperandeUnaire extends Operande {

	public OperandeUnaire() {
		this.exps = new ExpressionBooleenne[1];
	}
	
	public void setOperande(ExpressionBooleenne e) {
		this.exps[0] = e;
	}

}
