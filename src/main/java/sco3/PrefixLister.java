package sco3;

import static java.lang.System.out;

import software.amazon.awssdk.services.s3.S3Client;
import software.amazon.awssdk.services.s3.model.CommonPrefix;
import software.amazon.awssdk.services.s3.model.ListObjectsV2Request;
import software.amazon.awssdk.services.s3.paginators.ListObjectsV2Iterable;

public class PrefixLister {
	public static void main(String[] args) {
		S3Client s3 = S3Client.builder().build();

		ListObjectsV2Request lsReq = ListObjectsV2Request.builder() //
				.bucket("dz-bucket-1234") //
				.prefix("agent_") //
				.delimiter("/") //
				.build();

		ListObjectsV2Iterable listRes = s3.listObjectsV2Paginator(lsReq);
		for (CommonPrefix prefix : listRes.commonPrefixes()) {
			out.println(prefix.prefix());
		}
	}
}
