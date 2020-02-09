import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class GrilleBinaire {
	
	private int[] bin;
	
	public GrilleBinaire(int nb) {
		this.bin = new int[20];
		char[] c = Integer.toBinaryString(nb).toCharArray();
		List<Character> z = new ArrayList<>();
		for (char lol : c) z.add(lol);
		Collections.reverse(z);
		for (int i = 0; i < c.length; i++) {
			this.bin[i] = Character.getNumericValue(z.get(i));
		}
	}
	
	@Override
	public String toString() {
		String res = "[";
		for (int i : this.bin) {
			res += i+" "; 
		}
		res+="]";
		return res;
	}
	
	public int[] getBin() {
		return this.bin;
	}

}
