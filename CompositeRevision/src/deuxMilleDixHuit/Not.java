package deuxMilleDixHuit;

public class Not extends OperandeUnaire {
	
	public Not() {
		this.symbole = "!";
	}

	@Override
	public boolean evaluate() {
		return !this.exps[0].evaluate();
	}

}
